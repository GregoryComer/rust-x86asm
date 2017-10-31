use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 240], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 38], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 205], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1798693186, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 188, 147, 66, 225, 53, 107], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 208], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 30], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 249], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 43], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 33, 196], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 33, 26], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 125, 137, 33, 230], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM13)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 139, 33, 47], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 33, 215], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1452143899, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 33, 28, 253, 27, 245, 141, 86], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 125, 174, 33, 224], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1921328772, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 125, 175, 33, 12, 93, 132, 38, 133, 114], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 33, 196], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 33, 62], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 201, 33, 233], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectDisplaced(RAX, 634882239, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 206, 33, 152, 191, 136, 215, 37], OperandSize::Qword)
}

