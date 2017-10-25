use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vbroadcasti64x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X4, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(ECX, 629028377, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 91, 169, 25, 54, 126, 37], OperandSize::Dword)
}

fn vbroadcasti64x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI64X4, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 253, 205, 91, 60, 208], OperandSize::Qword)
}

