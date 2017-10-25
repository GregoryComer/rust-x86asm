use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn dppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 206, 101], OperandSize::Dword)
}

fn dppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 745920142, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 20, 141, 142, 214, 117, 44, 66], OperandSize::Dword)
}

fn dppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 197, 66], OperandSize::Qword)
}

fn dppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DPPD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 65, 16, 80], OperandSize::Qword)
}

