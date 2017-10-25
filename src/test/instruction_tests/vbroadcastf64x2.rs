use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vbroadcastf64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF64X2, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1208633944, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 26, 12, 133, 88, 74, 10, 72], OperandSize::Dword)
}

fn vbroadcastf64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF64X2, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 26, 4, 184], OperandSize::Qword)
}

