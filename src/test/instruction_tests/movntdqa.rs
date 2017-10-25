use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movntdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTDQA, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 2103610821, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 42, 44, 141, 197, 141, 98, 125], OperandSize::Dword)
}

fn movntdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTDQA, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1409998180, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 42, 36, 141, 100, 221, 10, 84], OperandSize::Qword)
}

