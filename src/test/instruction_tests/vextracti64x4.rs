use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vextracti64x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(Direct(YMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 204, 59, 214, 45], OperandSize::Dword)
}

fn vextracti64x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(IndirectDisplaced(ECX, 943769027, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 253, 72, 59, 169, 195, 197, 64, 56, 121], OperandSize::Dword)
}

fn vextracti64x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(Direct(YMM18)), operand2: Some(Direct(ZMM14)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 51, 253, 201, 59, 242, 50], OperandSize::Qword)
}

fn vextracti64x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1077548969, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 253, 72, 59, 140, 74, 169, 23, 58, 64, 36], OperandSize::Qword)
}

