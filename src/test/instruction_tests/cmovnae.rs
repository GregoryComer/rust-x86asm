use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovnae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 205], OperandSize::Word)
}

fn cmovnae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 32], OperandSize::Word)
}

fn cmovnae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(CX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 201], OperandSize::Dword)
}

fn cmovnae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 12, 83], OperandSize::Dword)
}

fn cmovnae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 247], OperandSize::Qword)
}

fn cmovnae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 12, 249], OperandSize::Qword)
}

fn cmovnae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 249], OperandSize::Word)
}

fn cmovnae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 151, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 136, 151, 0], OperandSize::Word)
}

fn cmovnae_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 238], OperandSize::Dword)
}

fn cmovnae_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 1121722415, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 156, 139, 47, 32, 220, 66], OperandSize::Dword)
}

fn cmovnae_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 234], OperandSize::Qword)
}

fn cmovnae_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1382741370, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 44, 125, 122, 245, 106, 82], OperandSize::Qword)
}

fn cmovnae_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 221], OperandSize::Qword)
}

fn cmovnae_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 2120967399, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 52, 69, 231, 100, 107, 126], OperandSize::Qword)
}

