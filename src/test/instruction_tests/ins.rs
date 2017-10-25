use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn ins_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 30061, Some(OperandSize::Byte), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[108], OperandSize::Word)
}

fn ins_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledDisplaced(ESI, Four, 1043632442, Some(OperandSize::Byte), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[108], OperandSize::Dword)
}

fn ins_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[108], OperandSize::Qword)
}

fn ins_4() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 23648, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[109], OperandSize::Word)
}

fn ins_5() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 523294221, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 109], OperandSize::Dword)
}

fn ins_6() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 109], OperandSize::Qword)
}

fn ins_7() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 76, Some(OperandSize::Dword), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 109], OperandSize::Word)
}

fn ins_8() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[109], OperandSize::Dword)
}

fn ins_9() {
    run_test(&Instruction { mnemonic: Mnemonic::INS, operand1: Some(IndirectScaledDisplaced(RDX, Two, 1798476112, Some(OperandSize::Dword), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[109], OperandSize::Qword)
}

