use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vbroadcastf32x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 25, 227], OperandSize::Dword)
}

fn vbroadcastf32x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(ECX, 1707411390, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 25, 153, 190, 7, 197, 101], OperandSize::Dword)
}

fn vbroadcastf32x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM29)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 125, 175, 25, 233], OperandSize::Qword)
}

fn vbroadcastf32x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(YMM8)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 172, 25, 1], OperandSize::Qword)
}

fn vbroadcastf32x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 25, 205], OperandSize::Dword)
}

fn vbroadcastf32x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 474296759, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 25, 164, 207, 183, 49, 69, 28], OperandSize::Dword)
}

fn vbroadcastf32x2_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 125, 204, 25, 231], OperandSize::Qword)
}

fn vbroadcastf32x2_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTF32X2, operand1: Some(Direct(ZMM9)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 207, 25, 12, 126], OperandSize::Qword)
}

