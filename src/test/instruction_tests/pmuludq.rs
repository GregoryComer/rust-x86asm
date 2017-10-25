use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmuludq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 207], OperandSize::Dword)
}

fn pmuludq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 46], OperandSize::Dword)
}

fn pmuludq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 201], OperandSize::Qword)
}

fn pmuludq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1590070936, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 20, 197, 152, 142, 198, 94], OperandSize::Qword)
}

fn pmuludq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 241], OperandSize::Dword)
}

fn pmuludq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 60, 127], OperandSize::Dword)
}

fn pmuludq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 220], OperandSize::Qword)
}

fn pmuludq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 1380280449, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 148, 142, 129, 104, 69, 82], OperandSize::Qword)
}

