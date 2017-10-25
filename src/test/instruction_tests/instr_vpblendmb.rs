use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 102, 217], OperandSize::Dword)
}

#[test]
fn vpblendmb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 973141880, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 143, 102, 186, 120, 247, 0, 58], OperandSize::Dword)
}

#[test]
fn vpblendmb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 13, 133, 102, 250], OperandSize::Qword)
}

#[test]
fn vpblendmb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM28)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 29, 133, 102, 34], OperandSize::Qword)
}

#[test]
fn vpblendmb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 102, 239], OperandSize::Dword)
}

#[test]
fn vpblendmb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 2105237642, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 170, 102, 60, 85, 138, 96, 123, 125], OperandSize::Dword)
}

#[test]
fn vpblendmb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 173, 102, 192], OperandSize::Qword)
}

#[test]
fn vpblendmb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 29, 173, 102, 44, 194], OperandSize::Qword)
}

#[test]
fn vpblendmb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 207, 102, 224], OperandSize::Dword)
}

#[test]
fn vpblendmb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(ECX, 487409391, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 206, 102, 177, 239, 70, 13, 29], OperandSize::Dword)
}

#[test]
fn vpblendmb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 117, 196, 102, 254], OperandSize::Qword)
}

#[test]
fn vpblendmb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 205, 102, 60, 177], OperandSize::Qword)
}

