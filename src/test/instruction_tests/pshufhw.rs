use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pshufhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 235, 34], OperandSize::Dword)
}

fn pshufhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 24, 104], OperandSize::Dword)
}

fn pshufhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 206, 91], OperandSize::Qword)
}

fn pshufhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 20, 155, 59], OperandSize::Qword)
}

