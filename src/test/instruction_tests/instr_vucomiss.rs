use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vucomiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 240], OperandSize::Dword)
}

#[test]
fn vucomiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDX, 2044816857, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 186, 217, 109, 225, 121], OperandSize::Dword)
}

#[test]
fn vucomiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 203], OperandSize::Qword)
}

#[test]
fn vucomiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1132278035, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 44, 253, 19, 49, 125, 67], OperandSize::Qword)
}

#[test]
fn vucomiss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 124, 24, 46, 216], OperandSize::Dword)
}

#[test]
fn vucomiss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 46, 20, 71], OperandSize::Dword)
}

#[test]
fn vucomiss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 129, 124, 24, 46, 200], OperandSize::Qword)
}

#[test]
fn vucomiss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISS, operand1: Some(Direct(XMM24)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 124, 8, 46, 7], OperandSize::Qword)
}

