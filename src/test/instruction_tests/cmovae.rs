use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 251], OperandSize::Word)
}

fn cmovae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 235, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 163, 235, 0], OperandSize::Word)
}

fn cmovae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 249], OperandSize::Dword)
}

fn cmovae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 52, 81], OperandSize::Dword)
}

fn cmovae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 243], OperandSize::Qword)
}

fn cmovae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(RBX, 2052847650, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 147, 34, 248, 91, 122], OperandSize::Qword)
}

fn cmovae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 253], OperandSize::Word)
}

fn cmovae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(ECX)), operand2: Some(IndirectDisplaced(DI, 23260, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 141, 220, 90], OperandSize::Word)
}

fn cmovae_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 253], OperandSize::Dword)
}

fn cmovae_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EDI)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 62], OperandSize::Dword)
}

fn cmovae_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 228], OperandSize::Qword)
}

fn cmovae_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RCX, 1488862142, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 145, 190, 59, 190, 88], OperandSize::Qword)
}

fn cmovae_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 245], OperandSize::Qword)
}

fn cmovae_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RDI, 1154507147, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 143, 139, 97, 208, 68], OperandSize::Qword)
}

