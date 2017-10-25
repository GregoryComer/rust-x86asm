use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn outs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 247, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[110], OperandSize::Word)
}

fn outs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(EDX, 1475107085, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[110], OperandSize::Dword)
}

fn outs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[110], OperandSize::Qword)
}

fn outs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[111], OperandSize::Word)
}

fn outs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 111], OperandSize::Dword)
}

fn outs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 111], OperandSize::Qword)
}

fn outs_7() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 19276, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 111], OperandSize::Word)
}

fn outs_8() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(EDI, 1170862218, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[111], OperandSize::Dword)
}

fn outs_9() {
    run_test(&Instruction { mnemonic: Mnemonic::OUTS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 110956227, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[111], OperandSize::Qword)
}

