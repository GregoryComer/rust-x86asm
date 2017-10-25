use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pslldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 252, 90], OperandSize::Dword)
}

fn pslldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLDQ, operand1: Some(Direct(XMM6)), operand2: Some(Literal8(32)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 254, 32], OperandSize::Qword)
}

