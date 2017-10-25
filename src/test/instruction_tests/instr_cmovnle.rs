use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(BP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 237], OperandSize::Word)
}

#[test]
fn cmovnle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 18], OperandSize::Word)
}

#[test]
fn cmovnle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 238], OperandSize::Dword)
}

#[test]
fn cmovnle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(EDI, 582606021, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 159, 197, 220, 185, 34], OperandSize::Dword)
}

#[test]
fn cmovnle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 215], OperandSize::Qword)
}

#[test]
fn cmovnle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1960792683, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 44, 181, 107, 82, 223, 116], OperandSize::Qword)
}

#[test]
fn cmovnle_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 253], OperandSize::Word)
}

#[test]
fn cmovnle_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EBX)), operand2: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 31], OperandSize::Word)
}

#[test]
fn cmovnle_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 210], OperandSize::Dword)
}

#[test]
fn cmovnle_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 1128869589, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 156, 198, 213, 46, 73, 67], OperandSize::Dword)
}

#[test]
fn cmovnle_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 229], OperandSize::Qword)
}

#[test]
fn cmovnle_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 33], OperandSize::Qword)
}

#[test]
fn cmovnle_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 255], OperandSize::Qword)
}

#[test]
fn cmovnle_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(RSP)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 35], OperandSize::Qword)
}

