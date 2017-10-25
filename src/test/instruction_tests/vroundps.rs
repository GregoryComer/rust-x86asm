use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vroundps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 248, 12], OperandSize::Dword)
}

fn vroundps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 52, 82, 50], OperandSize::Dword)
}

fn vroundps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 253, 108], OperandSize::Qword)
}

fn vroundps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 856002837, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 20, 141, 21, 145, 5, 51, 110], OperandSize::Qword)
}

fn vroundps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 195, 58], OperandSize::Dword)
}

fn vroundps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EDI, 556943935, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 175, 63, 74, 50, 33, 41], OperandSize::Dword)
}

fn vroundps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 231, 70], OperandSize::Qword)
}

fn vroundps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 2, 121], OperandSize::Qword)
}

