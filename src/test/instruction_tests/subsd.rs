use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn subsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 250], OperandSize::Dword)
}

fn subsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 2039312029, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 20, 253, 157, 110, 141, 121], OperandSize::Dword)
}

fn subsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 248], OperandSize::Qword)
}

fn subsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDI, 195897642, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 143, 42, 41, 173, 11], OperandSize::Qword)
}

