use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn bndldx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDLDX, operand1: Some(Direct(BND3)), operand2: Some(IndirectScaledIndexed(EDI, ECX, One, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 26, 28, 15], OperandSize::Dword)
}

fn bndldx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDLDX, operand1: Some(Direct(BND2)), operand2: Some(IndirectScaledIndexed(RBX, RBX, One, Some(OperandSize::Unsized), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 26, 20, 27], OperandSize::Qword)
}

