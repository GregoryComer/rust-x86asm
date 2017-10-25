use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vextractf32x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(Direct(XMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 206, 25, 249, 8], OperandSize::Dword)
}

fn vextractf32x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(IndirectScaledDisplaced(ECX, Two, 386155833, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 25, 28, 77, 57, 69, 4, 23, 76], OperandSize::Dword)
}

fn vextractf32x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(Direct(XMM22)), operand2: Some(Direct(ZMM31)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 35, 125, 203, 25, 254, 96], OperandSize::Qword)
}

fn vextractf32x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF32x4, operand1: Some(IndirectDisplaced(RDX, 1864987999, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM21)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 125, 72, 25, 170, 95, 117, 41, 111, 78], OperandSize::Qword)
}

