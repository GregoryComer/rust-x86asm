use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sar_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CL)), operand2: Some(Literal8(112)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 249, 112], OperandSize::Word)
}

#[test]
fn sar_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 2, Some(OperandSize::Byte), None)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 122, 2, 23], OperandSize::Word)
}

#[test]
fn sar_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CL)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 249, 92], OperandSize::Dword)
}

#[test]
fn sar_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 1552138710, Some(OperandSize::Byte), None)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 188, 199, 214, 193, 131, 92, 23], OperandSize::Dword)
}

#[test]
fn sar_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 250, 123], OperandSize::Qword)
}

#[test]
fn sar_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(RAX, Two, 1012544865, Some(OperandSize::Byte), None)), operand2: Some(Literal8(3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 60, 69, 97, 53, 90, 60, 3], OperandSize::Qword)
}

#[test]
fn sar_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BL)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 251, 16], OperandSize::Qword)
}

#[test]
fn sar_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1720165532, Some(OperandSize::Byte), None)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 60, 189, 156, 164, 135, 102, 52], OperandSize::Qword)
}

#[test]
fn sar_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(SP)), operand2: Some(Literal8(49)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 252, 49], OperandSize::Word)
}

#[test]
fn sar_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(DI, 35, Some(OperandSize::Word), None)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 125, 35, 58], OperandSize::Word)
}

#[test]
fn sar_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CX)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 249, 84], OperandSize::Dword)
}

#[test]
fn sar_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(ECX, Four, 1703524021, Some(OperandSize::Word), None)), operand2: Some(Literal8(5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 60, 141, 181, 182, 137, 101, 5], OperandSize::Dword)
}

#[test]
fn sar_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(SI)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 254, 41], OperandSize::Qword)
}

#[test]
fn sar_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 60, 211, 69], OperandSize::Qword)
}

#[test]
fn sar_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(59)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 253, 59], OperandSize::Word)
}

#[test]
fn sar_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 63, 2], OperandSize::Word)
}

#[test]
fn sar_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 252, 6], OperandSize::Dword)
}

#[test]
fn sar_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(EDI, Two, 459779031, Some(OperandSize::Dword), None)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 60, 125, 215, 171, 103, 27, 25], OperandSize::Dword)
}

#[test]
fn sar_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EDI)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 255, 24], OperandSize::Qword)
}

#[test]
fn sar_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(RDI, 1410793011, Some(OperandSize::Dword), None)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 191, 51, 254, 22, 84, 35], OperandSize::Qword)
}

#[test]
fn sar_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(RBP)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 253, 103], OperandSize::Qword)
}

#[test]
fn sar_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(RDX, 1080493636, Some(OperandSize::Qword), None)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 186, 68, 6, 103, 64, 34], OperandSize::Qword)
}

#[test]
fn sar_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 249], OperandSize::Word)
}

#[test]
fn sar_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 60], OperandSize::Word)
}

#[test]
fn sar_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 249], OperandSize::Dword)
}

#[test]
fn sar_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 62], OperandSize::Dword)
}

#[test]
fn sar_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 249], OperandSize::Qword)
}

#[test]
fn sar_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 60, 137], OperandSize::Qword)
}

#[test]
fn sar_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 250], OperandSize::Qword)
}

#[test]
fn sar_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 60, 185], OperandSize::Qword)
}

#[test]
fn sar_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 251], OperandSize::Word)
}

#[test]
fn sar_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(SI, 183, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 188, 183, 0], OperandSize::Word)
}

#[test]
fn sar_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 253], OperandSize::Dword)
}

#[test]
fn sar_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(ESI, 393646315, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 190, 235, 144, 118, 23], OperandSize::Dword)
}

#[test]
fn sar_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(SI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 254], OperandSize::Qword)
}

#[test]
fn sar_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(RAX, Four, 720539427, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 60, 133, 35, 143, 242, 42], OperandSize::Qword)
}

#[test]
fn sar_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 253], OperandSize::Word)
}

#[test]
fn sar_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 18, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 123, 18], OperandSize::Word)
}

#[test]
fn sar_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 254], OperandSize::Dword)
}

#[test]
fn sar_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 60, 153], OperandSize::Dword)
}

#[test]
fn sar_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 254], OperandSize::Qword)
}

#[test]
fn sar_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 678644365, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 188, 176, 141, 74, 115, 40], OperandSize::Qword)
}

#[test]
fn sar_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(RDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 255], OperandSize::Qword)
}

#[test]
fn sar_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(RSI, 1019918588, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 190, 252, 184, 202, 60], OperandSize::Qword)
}

#[test]
fn sar_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 249], OperandSize::Word)
}

#[test]
fn sar_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(SI, 28018, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 188, 114, 109], OperandSize::Word)
}

#[test]
fn sar_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 249], OperandSize::Dword)
}

#[test]
fn sar_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 531233436, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 188, 73, 156, 250, 169, 31], OperandSize::Dword)
}

#[test]
fn sar_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 251], OperandSize::Qword)
}

#[test]
fn sar_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 63], OperandSize::Qword)
}

#[test]
fn sar_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 250], OperandSize::Qword)
}

#[test]
fn sar_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1194115914, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 60, 77, 74, 195, 44, 71], OperandSize::Qword)
}

#[test]
fn sar_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(SI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 254], OperandSize::Word)
}

#[test]
fn sar_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(DI, 24704, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 189, 128, 96], OperandSize::Word)
}

#[test]
fn sar_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(SP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 252], OperandSize::Dword)
}

#[test]
fn sar_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 59], OperandSize::Dword)
}

#[test]
fn sar_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 249], OperandSize::Qword)
}

#[test]
fn sar_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 60, 200], OperandSize::Qword)
}

#[test]
fn sar_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EBX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 251], OperandSize::Word)
}

#[test]
fn sar_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 13386, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 187, 74, 52], OperandSize::Word)
}

#[test]
fn sar_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(ECX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 249], OperandSize::Dword)
}

#[test]
fn sar_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 1589652347, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 188, 192, 123, 43, 192, 94], OperandSize::Dword)
}

#[test]
fn sar_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EBX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 251], OperandSize::Qword)
}

#[test]
fn sar_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 1389129355, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 60, 253, 139, 110, 204, 82], OperandSize::Qword)
}

#[test]
fn sar_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(RBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 253], OperandSize::Qword)
}

#[test]
fn sar_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 63], OperandSize::Qword)
}

