use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pshuflw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 251, 72], OperandSize::Dword)
}

fn pshuflw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 766391898, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 164, 130, 90, 54, 174, 45, 95], OperandSize::Dword)
}

fn pshuflw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 218, 75], OperandSize::Qword)
}

fn pshuflw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFLW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 414144084, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 112, 4, 253, 84, 86, 175, 24, 116], OperandSize::Qword)
}

