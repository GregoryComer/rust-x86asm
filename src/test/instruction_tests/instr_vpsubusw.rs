use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 217, 240], OperandSize::Dword)
}

#[test]
fn vpsubusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 1886562218, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 217, 146, 170, 167, 114, 112], OperandSize::Dword)
}

#[test]
fn vpsubusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 217, 210], OperandSize::Qword)
}

#[test]
fn vpsubusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 217, 57], OperandSize::Qword)
}

#[test]
fn vpsubusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 217, 197], OperandSize::Dword)
}

#[test]
fn vpsubusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 217, 28, 185], OperandSize::Dword)
}

#[test]
fn vpsubusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 217, 255], OperandSize::Qword)
}

#[test]
fn vpsubusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RBX, 1857598829, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 217, 163, 109, 181, 184, 110], OperandSize::Qword)
}

#[test]
fn vpsubusw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 142, 217, 226], OperandSize::Dword)
}

#[test]
fn vpsubusw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 140, 217, 55], OperandSize::Dword)
}

#[test]
fn vpsubusw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 101, 129, 217, 197], OperandSize::Qword)
}

#[test]
fn vpsubusw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 134, 217, 51], OperandSize::Qword)
}

#[test]
fn vpsubusw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 175, 217, 227], OperandSize::Dword)
}

#[test]
fn vpsubusw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1012973528, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 173, 217, 180, 152, 216, 191, 96, 60], OperandSize::Dword)
}

#[test]
fn vpsubusw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 69, 171, 217, 201], OperandSize::Qword)
}

#[test]
fn vpsubusw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSW, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1148596441, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 13, 166, 217, 172, 187, 217, 48, 118, 68], OperandSize::Qword)
}

