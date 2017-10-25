use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn rcpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 249], OperandSize::Dword)
}

fn rcpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 36, 86], OperandSize::Dword)
}

fn rcpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 200], OperandSize::Qword)
}

fn rcpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 306966954, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 83, 180, 66, 170, 241, 75, 18], OperandSize::Qword)
}

