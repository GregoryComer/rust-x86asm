use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcmpsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(73)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 219, 194, 245, 73], OperandSize::Dword)
}

fn vcmpsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1301516535, Some(OperandSize::Qword), None)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 194, 20, 205, 247, 144, 147, 77, 2], OperandSize::Dword)
}

fn vcmpsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 194, 205, 53], OperandSize::Qword)
}

fn vcmpsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 1369234806, Some(OperandSize::Qword), None)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 194, 156, 191, 118, 221, 156, 81, 4], OperandSize::Qword)
}

fn vcmpsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 31, 194, 243, 47], OperandSize::Dword)
}

fn vcmpsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 231, 10, 194, 28, 138, 64], OperandSize::Dword)
}

fn vcmpsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM20)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 231, 27, 194, 220, 30], OperandSize::Qword)
}

fn vcmpsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 207, 4, 194, 60, 209, 53], OperandSize::Qword)
}

