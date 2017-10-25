use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn xsavec64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC64, operand1: Some(IndirectScaledDisplaced(RAX, Four, 2128812417, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 199, 36, 133, 129, 25, 227, 126], OperandSize::Qword)
}

