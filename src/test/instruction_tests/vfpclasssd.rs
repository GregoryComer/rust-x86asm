use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfpclasssd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 11, 103, 212, 5], OperandSize::Dword)
}

fn vfpclasssd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 153484929, Some(OperandSize::Qword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 11, 103, 60, 213, 129, 254, 37, 9, 47], OperandSize::Dword)
}

fn vfpclasssd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM16)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 179, 253, 10, 103, 208, 38], OperandSize::Qword)
}

fn vfpclasssd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSSD, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 9, 103, 20, 214, 3], OperandSize::Qword)
}

