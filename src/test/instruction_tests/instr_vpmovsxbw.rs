use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 233], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1021667121, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 140, 208, 49, 103, 229, 60], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 238], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 1844897310, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 188, 91, 30, 230, 246, 109], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 203], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 44, 70], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 197], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 219537252, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 172, 134, 100, 223, 21, 13], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 32, 246], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 241415119, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 32, 180, 223, 207, 179, 99, 14], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 32, 214], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM20)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 3322038, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 139, 32, 36, 133, 182, 176, 50, 0], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 32, 234], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1631343089, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 32, 28, 181, 241, 81, 60, 97], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 210, 125, 170, 32, 203], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM23)), operand2: Some(IndirectDisplaced(RDI, 628561867, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 173, 32, 191, 203, 23, 119, 37], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 32, 249], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(ESI, 456303356, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 32, 158, 252, 162, 50, 27], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 204, 32, 250], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 351932654, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 32, 148, 240, 238, 16, 250, 20], OperandSize::Qword)
}

