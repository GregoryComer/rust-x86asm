use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn xsaves64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVES64, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 1507884545, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 199, 172, 83, 1, 126, 224, 89], OperandSize::Qword)
}

