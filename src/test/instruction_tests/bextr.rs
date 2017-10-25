use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn bextr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 104, 247, 215], OperandSize::Dword)
}

fn bextr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 104, 247, 12, 247], OperandSize::Dword)
}

fn bextr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 247, 207], OperandSize::Qword)
}

fn bextr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(ESI)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 247, 50], OperandSize::Qword)
}

fn bextr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSP)), operand3: Some(Direct(RDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 232, 247, 244], OperandSize::Qword)
}

fn bextr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1688700450, Some(OperandSize::Qword), None)), operand3: Some(Direct(RCX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 240, 247, 140, 82, 34, 134, 167, 100], OperandSize::Qword)
}

