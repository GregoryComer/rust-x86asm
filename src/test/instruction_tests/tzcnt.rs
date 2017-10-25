use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn tzcnt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(BX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 222], OperandSize::Dword)
}

fn tzcnt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 36, 243], OperandSize::Dword)
}

fn tzcnt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 205], OperandSize::Qword)
}

fn tzcnt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(CX)), operand2: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 243, 15, 188, 11], OperandSize::Qword)
}

fn tzcnt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 220], OperandSize::Dword)
}

fn tzcnt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 976637000, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 156, 139, 72, 76, 54, 58], OperandSize::Dword)
}

fn tzcnt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 234], OperandSize::Qword)
}

fn tzcnt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1544277733, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 188, 52, 221, 229, 206, 11, 92], OperandSize::Qword)
}

fn tzcnt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 188, 255], OperandSize::Qword)
}

fn tzcnt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::TZCNT, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 478828077, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 188, 12, 197, 45, 86, 138, 28], OperandSize::Qword)
}

