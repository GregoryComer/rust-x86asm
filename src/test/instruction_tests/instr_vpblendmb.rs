use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 137, 102, 214], OperandSize::Dword)
}

#[test]
fn vpblendmb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 224585413, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 143, 102, 184, 197, 230, 98, 13], OperandSize::Dword)
}

#[test]
fn vpblendmb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 13, 131, 102, 226], OperandSize::Qword)
}

#[test]
fn vpblendmb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 690775277, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 37, 132, 102, 52, 117, 237, 100, 44, 41], OperandSize::Qword)
}

#[test]
fn vpblendmb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 170, 102, 221], OperandSize::Dword)
}

#[test]
fn vpblendmb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1871984231, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 102, 4, 221, 103, 54, 148, 111], OperandSize::Dword)
}

#[test]
fn vpblendmb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 53, 171, 102, 225], OperandSize::Qword)
}

#[test]
fn vpblendmb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 1188174214, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 37, 169, 102, 132, 131, 134, 25, 210, 70], OperandSize::Qword)
}

#[test]
fn vpblendmb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 201, 102, 244], OperandSize::Dword)
}

#[test]
fn vpblendmb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(ESI, 864129096, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 206, 102, 134, 72, 144, 129, 51], OperandSize::Dword)
}

#[test]
fn vpblendmb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 5, 199, 102, 239], OperandSize::Qword)
}

#[test]
fn vpblendmb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM20)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 93, 195, 102, 58], OperandSize::Qword)
}

