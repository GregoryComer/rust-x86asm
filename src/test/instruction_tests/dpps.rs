use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn dpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 249, 45], OperandSize::Dword)
}

fn dpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1564821901, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 12, 221, 141, 73, 69, 93, 4], OperandSize::Dword)
}

fn dpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 206, 107], OperandSize::Qword)
}

fn dpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 64, 30, 87], OperandSize::Qword)
}

