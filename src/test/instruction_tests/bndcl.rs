use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn bndcl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND0)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 193], OperandSize::Dword)
}

fn bndcl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND0)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 178636494, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 4, 157, 206, 198, 165, 10], OperandSize::Dword)
}

fn bndcl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND0)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 198], OperandSize::Qword)
}

fn bndcl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND0)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1824817430, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 4, 213, 22, 129, 196, 108], OperandSize::Qword)
}

