use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 233], OperandSize::Word)
}

fn cmovle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(DI, 19, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 101, 19], OperandSize::Word)
}

fn cmovle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 249], OperandSize::Dword)
}

fn cmovle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(DI)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 56], OperandSize::Dword)
}

fn cmovle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 211], OperandSize::Qword)
}

fn cmovle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(BP)), operand2: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 40], OperandSize::Qword)
}

fn cmovle_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 205], OperandSize::Word)
}

fn cmovle_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ESP)), operand2: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 36], OperandSize::Word)
}

fn cmovle_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 254], OperandSize::Dword)
}

fn cmovle_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1844986404, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 140, 134, 36, 66, 248, 109], OperandSize::Dword)
}

fn cmovle_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 243], OperandSize::Qword)
}

fn cmovle_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 429199924, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 164, 118, 52, 18, 149, 25], OperandSize::Qword)
}

fn cmovle_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(RBX)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 217], OperandSize::Qword)
}

fn cmovle_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVLE, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 669428874, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 164, 193, 138, 172, 230, 39], OperandSize::Qword)
}

