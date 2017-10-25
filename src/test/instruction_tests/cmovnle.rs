use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovnle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 204], OperandSize::Word)
}

fn cmovnle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(DI, 249, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 173, 249, 0], OperandSize::Word)
}

fn cmovnle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 204], OperandSize::Dword)
}

fn cmovnle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(CX)), operand2: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 10], OperandSize::Dword)
}

fn cmovnle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 238], OperandSize::Qword)
}

fn cmovnle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 804964089, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 20, 149, 249, 198, 250, 47], OperandSize::Qword)
}

fn cmovnle_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 229], OperandSize::Word)
}

fn cmovnle_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 16887, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 79, 163, 247, 65], OperandSize::Word)
}

fn cmovnle_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 239], OperandSize::Dword)
}

fn cmovnle_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1704013400, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 140, 191, 88, 46, 145, 101], OperandSize::Dword)
}

fn cmovnle_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 203], OperandSize::Qword)
}

fn cmovnle_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 79, 28, 139], OperandSize::Qword)
}

fn cmovnle_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(RBX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 218], OperandSize::Qword)
}

fn cmovnle_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNLE, operand1: Some(Direct(RDX)), operand2: Some(IndirectDisplaced(RSI, 347657120, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 79, 150, 160, 211, 184, 20], OperandSize::Qword)
}

