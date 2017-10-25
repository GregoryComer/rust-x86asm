use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movmskpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVMSKPD, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 80, 225], OperandSize::Dword)
}

fn movmskpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVMSKPD, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 80, 209], OperandSize::Qword)
}

