use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pcmpistri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 229, 93], OperandSize::Dword)
}

fn pcmpistri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 2015470100, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 140, 79, 20, 162, 33, 120, 102], OperandSize::Dword)
}

fn pcmpistri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 254, 17], OperandSize::Qword)
}

fn pcmpistri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPISTRI, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 610671191, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 99, 12, 117, 87, 26, 102, 36, 51], OperandSize::Qword)
}

