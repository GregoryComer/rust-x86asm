use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vbroadcasti128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI128, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1526305989, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 90, 20, 69, 197, 148, 249, 90], OperandSize::Dword)
}

fn vbroadcasti128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI128, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 90, 23], OperandSize::Qword)
}

