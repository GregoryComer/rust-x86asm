use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn xorps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 212], OperandSize::Dword)
}

fn xorps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EDX, 1461441167, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 154, 143, 210, 27, 87], OperandSize::Dword)
}

fn xorps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 234], OperandSize::Qword)
}

fn xorps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 87, 44, 247], OperandSize::Qword)
}

