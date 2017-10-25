use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn lea_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(SP)), operand2: Some(Indirect(DI, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[141, 37], OperandSize::Word)
}

fn lea_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 674193514, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 141, 148, 86, 106, 96, 47, 40], OperandSize::Dword)
}

fn lea_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(RDI, 1111731427, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 141, 175, 227, 172, 67, 66], OperandSize::Qword)
}

fn lea_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 141, 8], OperandSize::Word)
}

fn lea_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(EDX, 1709919116, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[141, 154, 140, 75, 235, 101], OperandSize::Dword)
}

fn lea_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(RCX, 551438069, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[141, 185, 245, 70, 222, 32], OperandSize::Qword)
}

fn lea_7() {
    run_test(&Instruction { mnemonic: Mnemonic::LEA, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 244959761, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 141, 156, 136, 17, 202, 153, 14], OperandSize::Qword)
}

