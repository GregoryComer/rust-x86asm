use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vextractf64x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(Direct(YMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 206, 27, 219, 106], OperandSize::Dword)
}

fn vextractf64x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(IndirectDisplaced(ECX, 572854929, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 253, 72, 27, 185, 145, 18, 37, 34, 55], OperandSize::Dword)
}

fn vextractf64x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(Direct(YMM31)), operand2: Some(Direct(ZMM31)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 3, 253, 202, 27, 255, 102], OperandSize::Qword)
}

fn vextractf64x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 1265595217, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM18)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 253, 72, 27, 20, 253, 81, 115, 111, 75, 91], OperandSize::Qword)
}

