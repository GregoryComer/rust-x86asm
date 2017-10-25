use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn blendvpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 203], OperandSize::Dword)
}

fn blendvpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 44, 216], OperandSize::Dword)
}

fn blendvpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 235], OperandSize::Qword)
}

fn blendvpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1355058573, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 28, 197, 141, 141, 196, 80], OperandSize::Qword)
}

