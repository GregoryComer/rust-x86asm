use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn mulps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 209], OperandSize::Dword)
}

fn mulps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 991414085, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 4, 253, 69, 199, 23, 59], OperandSize::Dword)
}

fn mulps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 227], OperandSize::Qword)
}

fn mulps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 2144972213, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 89, 60, 245, 181, 173, 217, 127], OperandSize::Qword)
}

