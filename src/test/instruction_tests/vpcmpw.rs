use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 237, 13, 63, 233, 103], OperandSize::Dword)
}

fn vpcmpw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1494892907, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 197, 13, 63, 148, 119, 107, 65, 26, 89, 77], OperandSize::Dword)
}

fn vpcmpw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM25)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 147, 149, 3, 63, 233, 7], OperandSize::Qword)
}

fn vpcmpw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 173, 14, 63, 44, 90, 124], OperandSize::Qword)
}

fn vpcmpw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 205, 44, 63, 215, 103], OperandSize::Dword)
}

fn vpcmpw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 197, 47, 63, 36, 91, 30], OperandSize::Dword)
}

fn vpcmpw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 133, 35, 63, 210, 17], OperandSize::Qword)
}

fn vpcmpw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1250054873, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 141, 37, 63, 148, 123, 217, 82, 130, 74, 42], OperandSize::Qword)
}

fn vpcmpw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(107)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 197, 78, 63, 200, 107], OperandSize::Dword)
}

fn vpcmpw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 197, 75, 63, 58, 61], OperandSize::Dword)
}

fn vpcmpw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM13)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 211, 213, 71, 63, 245, 102], OperandSize::Qword)
}

fn vpcmpw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectDisplaced(RCX, 1502089349, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 165, 74, 63, 137, 133, 16, 136, 89, 15], OperandSize::Qword)
}

