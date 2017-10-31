use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM3)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 222], OperandSize::Dword)
}

#[test]
fn movd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 46], OperandSize::Dword)
}

#[test]
fn movd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM7)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 255], OperandSize::Qword)
}

#[test]
fn movd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 52, 202], OperandSize::Qword)
}

#[test]
fn movd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 217], OperandSize::Dword)
}

#[test]
fn movd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 44, 65], OperandSize::Dword)
}

#[test]
fn movd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 242], OperandSize::Qword)
}

#[test]
fn movd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 26], OperandSize::Qword)
}

#[test]
fn movd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(ECX)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 217], OperandSize::Dword)
}

#[test]
fn movd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectDisplaced(EDX, 822672813, Some(OperandSize::Dword), None)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 170, 173, 253, 8, 49], OperandSize::Dword)
}

#[test]
fn movd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EDX)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 242], OperandSize::Qword)
}

#[test]
fn movd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1698680169, Some(OperandSize::Dword), None)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 60, 205, 105, 205, 63, 101], OperandSize::Qword)
}

#[test]
fn movd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 199], OperandSize::Dword)
}

#[test]
fn movd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectDisplaced(EAX, 1844348236, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 168, 76, 133, 238, 109], OperandSize::Dword)
}

#[test]
fn movd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 202], OperandSize::Qword)
}

#[test]
fn movd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 6], OperandSize::Qword)
}

