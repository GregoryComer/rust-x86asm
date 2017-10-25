use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn punpcklbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 212], OperandSize::Dword)
}

fn punpcklbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 986377856, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 132, 95, 128, 238, 202, 58], OperandSize::Dword)
}

fn punpcklbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 243], OperandSize::Qword)
}

fn punpcklbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 1784160107, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 188, 128, 107, 31, 88, 106], OperandSize::Qword)
}

fn punpcklbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 237], OperandSize::Dword)
}

fn punpcklbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ECX, 1688497116, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 177, 220, 107, 164, 100], OperandSize::Dword)
}

fn punpcklbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 202], OperandSize::Qword)
}

fn punpcklbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1459074657, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 140, 126, 97, 182, 247, 86], OperandSize::Qword)
}

