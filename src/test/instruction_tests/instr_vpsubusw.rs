use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 217, 199], OperandSize::Dword)
}

#[test]
fn vpsubusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 217, 44, 183], OperandSize::Dword)
}

#[test]
fn vpsubusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 217, 255], OperandSize::Qword)
}

#[test]
fn vpsubusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 1062239769, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 217, 132, 95, 25, 126, 80, 63], OperandSize::Qword)
}

#[test]
fn vpsubusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 217, 234], OperandSize::Dword)
}

#[test]
fn vpsubusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 217, 48], OperandSize::Dword)
}

#[test]
fn vpsubusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 217, 252], OperandSize::Qword)
}

#[test]
fn vpsubusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1456783341, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 217, 20, 149, 237, 191, 212, 86], OperandSize::Qword)
}

#[test]
fn vpsubusw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 138, 217, 220], OperandSize::Dword)
}

#[test]
fn vpsubusw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDI, 1011293422, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 138, 217, 183, 238, 28, 71, 60], OperandSize::Dword)
}

#[test]
fn vpsubusw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 117, 143, 217, 208], OperandSize::Qword)
}

#[test]
fn vpsubusw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 45, 137, 217, 55], OperandSize::Qword)
}

#[test]
fn vpsubusw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 169, 217, 204], OperandSize::Dword)
}

#[test]
fn vpsubusw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 175, 217, 24], OperandSize::Dword)
}

#[test]
fn vpsubusw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 45, 167, 217, 232], OperandSize::Qword)
}

#[test]
fn vpsubusw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectDisplaced(RSI, 642571584, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 5, 169, 217, 134, 64, 221, 76, 38], OperandSize::Qword)
}

