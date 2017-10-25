use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovna_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 236], OperandSize::Word)
}

#[test]
fn cmovna_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 32213, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 184, 213, 125], OperandSize::Word)
}

#[test]
fn cmovna_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 254], OperandSize::Dword)
}

#[test]
fn cmovna_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 468889605, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 52, 245, 5, 176, 242, 27], OperandSize::Dword)
}

#[test]
fn cmovna_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(BP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 237], OperandSize::Qword)
}

#[test]
fn cmovna_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 60, 119], OperandSize::Qword)
}

#[test]
fn cmovna_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 243], OperandSize::Word)
}

#[test]
fn cmovna_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 226, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 70, 178, 226, 0], OperandSize::Word)
}

#[test]
fn cmovna_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 215], OperandSize::Dword)
}

#[test]
fn cmovna_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(EDI, 522681001, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 175, 169, 122, 39, 31], OperandSize::Dword)
}

#[test]
fn cmovna_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 247], OperandSize::Qword)
}

#[test]
fn cmovna_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 234070194, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 70, 12, 125, 178, 160, 243, 13], OperandSize::Qword)
}

#[test]
fn cmovna_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 246], OperandSize::Qword)
}

#[test]
fn cmovna_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNA, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RCX, 125803593, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 70, 137, 73, 156, 127, 7], OperandSize::Qword)
}

