use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn lzcnt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 210], OperandSize::Word)
}

fn lzcnt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(BP)), operand2: Some(Indirect(BX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 47], OperandSize::Word)
}

fn lzcnt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 214], OperandSize::Dword)
}

fn lzcnt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(EAX, 744469333, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 152, 85, 179, 95, 44], OperandSize::Dword)
}

fn lzcnt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 253], OperandSize::Qword)
}

fn lzcnt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(CX)), operand2: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 8], OperandSize::Qword)
}

fn lzcnt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 222], OperandSize::Word)
}

fn lzcnt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 189, 58], OperandSize::Word)
}

fn lzcnt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 238], OperandSize::Dword)
}

fn lzcnt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 20, 95], OperandSize::Dword)
}

fn lzcnt_11() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 247], OperandSize::Qword)
}

fn lzcnt_12() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(EBX)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 189, 27], OperandSize::Qword)
}

fn lzcnt_13() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(RDX)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 189, 209], OperandSize::Qword)
}

fn lzcnt_14() {
    run_test(&Instruction { mnemonic: Mnemonic::LZCNT, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 189, 36, 251], OperandSize::Qword)
}

