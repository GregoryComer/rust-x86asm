use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn aesdec_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDEC, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 222, 230], OperandSize::Dword)
}

fn aesdec_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDEC, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 454573889, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 222, 28, 205, 65, 63, 24, 27], OperandSize::Dword)
}

fn aesdec_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDEC, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 222, 232], OperandSize::Qword)
}

fn aesdec_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDEC, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 222, 28, 242], OperandSize::Qword)
}

