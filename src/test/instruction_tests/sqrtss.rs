use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 205], OperandSize::Dword)
}

fn sqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 885216012, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 60, 77, 12, 83, 195, 52], OperandSize::Dword)
}

fn sqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 245], OperandSize::Qword)
}

fn sqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 63], OperandSize::Qword)
}

