use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn aesenclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 202], OperandSize::Dword)
}

fn aesenclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EBX, 1061773897, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 155, 73, 98, 73, 63], OperandSize::Dword)
}

fn aesenclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 246], OperandSize::Qword)
}

fn aesenclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 2], OperandSize::Qword)
}

