use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 233, 198], OperandSize::Dword)
}

#[test]
fn vpsubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 233, 17], OperandSize::Dword)
}

#[test]
fn vpsubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 233, 234], OperandSize::Qword)
}

#[test]
fn vpsubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RAX, 410623705, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 233, 152, 217, 158, 121, 24], OperandSize::Qword)
}

#[test]
fn vpsubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 233, 205], OperandSize::Dword)
}

#[test]
fn vpsubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 233, 59], OperandSize::Dword)
}

#[test]
fn vpsubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 233, 233], OperandSize::Qword)
}

#[test]
fn vpsubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RDX, 122319364, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 233, 138, 4, 114, 74, 7], OperandSize::Qword)
}

#[test]
fn vpsubsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 140, 233, 250], OperandSize::Dword)
}

#[test]
fn vpsubsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 143, 233, 36, 67], OperandSize::Dword)
}

#[test]
fn vpsubsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 125, 140, 233, 248], OperandSize::Qword)
}

#[test]
fn vpsubsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1905947076, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 77, 141, 233, 180, 250, 196, 113, 154, 113], OperandSize::Qword)
}

#[test]
fn vpsubsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 169, 233, 233], OperandSize::Dword)
}

#[test]
fn vpsubsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 1038450861, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 174, 233, 137, 173, 128, 229, 61], OperandSize::Dword)
}

#[test]
fn vpsubsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 53, 171, 233, 237], OperandSize::Qword)
}

#[test]
fn vpsubsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1510011786, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 77, 167, 233, 36, 69, 138, 243, 0, 90], OperandSize::Qword)
}

