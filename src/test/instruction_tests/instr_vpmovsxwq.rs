use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 232], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EDX, 831371787, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 130, 11, 186, 141, 49], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 195], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1278658398, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 4, 157, 94, 199, 54, 76], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 241], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 4, 190], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 240], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 31], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 36, 221], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 36, 4, 88], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 125, 139, 36, 210], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1351533215, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 36, 20, 85, 159, 194, 142, 80], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 36, 210], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 36, 28, 249], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 125, 169, 36, 253], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 36, 62], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 36, 202], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(ESI, 1118314430, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 36, 150, 190, 31, 168, 66], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 125, 203, 36, 194], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 158883573, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 36, 12, 213, 245, 94, 120, 9], OperandSize::Qword)
}

