use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vdpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 64, 207, 68], OperandSize::Dword)
}

fn vdpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 92361815, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 64, 164, 182, 87, 84, 129, 5, 92], OperandSize::Dword)
}

fn vdpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 64, 231, 39], OperandSize::Qword)
}

fn vdpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 1141114854, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 64, 140, 135, 230, 7, 4, 68, 10], OperandSize::Qword)
}

fn vdpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 64, 213, 111], OperandSize::Dword)
}

fn vdpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1372680476, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 64, 12, 157, 28, 113, 209, 81, 65], OperandSize::Dword)
}

fn vdpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 64, 238, 101], OperandSize::Qword)
}

fn vdpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDPPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1538227896, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 64, 52, 125, 184, 126, 175, 91, 82], OperandSize::Qword)
}

