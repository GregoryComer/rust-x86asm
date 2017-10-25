use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vbroadcastf128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF128, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(ECX, 1177302421, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 26, 137, 149, 53, 44, 70], OperandSize::Dword)
}

fn vbroadcastf128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF128, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1380445909, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 26, 12, 85, 213, 238, 71, 82], OperandSize::Qword)
}

